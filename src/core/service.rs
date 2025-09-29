#[warn(unused_imports)]
use crate::core::{
    model::User,
    repository::UserRepository,
    schema::{CreateUser, UpdateUser},
};
use anyhow::Result;
use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use base32;
use chrono::Utc;
use otpauth::TOTP;
use password_hash::rand_core::OsRng;
use rand::Rng;
use sqlx::PgPool;
//use validator::Validate;

use crate::{
    core::schema::{UpdateUserPassword},
    repository::{PaginatedResponse, Repository},
};

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            repo: UserRepository,
        }
    }

    pub async fn get_by_id(&self, pool: &PgPool, id: i64) -> Result<User> {
        Ok(self.repo.get_by_id(pool, id).await?)
    }

    pub async fn create(&self, pool: &PgPool, input: CreateUser) -> Result<User> {
        let mut input = input;

        input.password = Self::get_password_hash(&Self::random_base32().to_string())
            .unwrap_or("NovaSenhaTeste!!####".to_string());
        //input.otp_base32 = Some(Self::random_base32());

        Ok(self.repo.create(pool, input).await?)
    }

    pub async fn update(&self, pool: &PgPool, id: i64, input: UpdateUser) -> Result<User> {
        // 1. Buscar usuário atual
        //let current = self.repo.get_by_id(pool, id).await?;

        // 2. Mesclar dados novos com atuais
        //let updated_user = Self::apply_to(&current, input);

        Ok(self.repo.update(pool, id, input).await?)
    }

    pub async fn delete(&self, pool: &PgPool, id: i64) -> Result<()> {
        Ok(self.repo.delete(pool, id).await?)
    }

    /*
       verifica se usuario tem permissao
    */
    /* pub async fn have_permission(user: &User, permission: String, db: &PgPool) -> bool {
        // Verifica se o usuário é um superusuário
        if user.is_superuser {
            return true;
        }

        //pegar todas as permissões pelos perfies do usuario.
        let list_permissao = match UserRolesService::get_user_permissions(&db, user.id).await {
            Ok(permissions) => permissions,
            Err(_) => return false, // Em caso de erro, nega acesso
        };

        for perm in list_permissao {
            // Se tiver permissão de admin, liberar tudo
            if permission.eq_ignore_ascii_case(&perm) {
            } else {
                continue;
            }
        }

        false
    } */

    /*
    retorna todas as permissões de um usuário
    */
    /* pub async fn get_user_permissions(db: &PgPool, user_id: i64) -> Vec<String> {
        let list_permissao = match UserRolesService::get_user_permissions(&db, user_id).await {
            Ok(permissions) => permissions,
            Err(_) => return vec![],
        };
        list_permissao
    } */

    pub async fn get_paginated(
        &self,
        pool: &PgPool,
        find: Option<&str>,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<User>> {
        Ok(self
            .repo
            .get_paginated(pool, find, page, page_size, None)
            .await?)
    }

    pub fn get_password_hash(password: &str) -> Result<String, password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);

        // Configura Argon2id com parâmetros recomendados (OWASP 2025)
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15_000, 2, 1, None).unwrap(), // memória KB, iterações, paralelismo
        );

        Ok(argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string())
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, password_hash::Error> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Gera um segredo aleatório em Base32 para OTP
    pub fn random_base32() -> String {
        let mut rng = rand::rng();
        let bytes: Vec<u8> = (0..20).map(|_| rng.random()).collect();
        base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &bytes)
    }

    /// Valida o OTP com suporte a fuso horário de São Paulo
    pub fn is_valid_otp(otp: &str, otp_base32: &str) -> bool {
        //let secret_base32 = "WLPZ6PWJNA5VE5XPV3EC3G77H5MVPJMI";
        let secret_base32 = otp_base32;

        // Criar TOTP no padrão do Google Authenticator:
        // - SHA1
        // - 6 dígitos
        // - Intervalo de 30 segundos
        let totp = TOTP::from_base32(secret_base32).unwrap();

        // Gerar código atual
        //let code = totp.generate(30, Utc::now().timestamp() as u64);
        //println!("Código gerado: {}", code);

        // Converter entrada do cliente
        let codigo: u32 = otp.parse().unwrap();

        //debug!("Código gerado: {}, codigo enviado: {}", code, codigo);

        // Verificar
        totp.verify(codigo, 30, Utc::now().timestamp() as u64)
    }

    pub fn gerar_otp(otp_base32: &str) -> String {
        let totp = TOTP::new(otp_base32.to_string());
        totp.generate(30, Utc::now().timestamp() as u64).to_string()
    }

    pub fn get_otp_url(username: &str, otp_base32: &str) -> String {
        let totp = TOTP::new(otp_base32.to_string());
        totp.to_uri("YourApp", username)
    }

    pub async fn update_password(
        pool: &PgPool,
        id: i64,
        input: UpdateUserPassword,
    ) -> Result<User> {
        let password = Self::get_password_hash(&input.password).unwrap();

        Ok(sqlx::query_as!(
            User,
            r#"
            UPDATE auth_user
            SET 
                password = $1
            WHERE id = $2
            RETURNING *
            "#,
            password,
            id as i32
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<User> {
        let query = format!("SELECT * FROM users WHERE username = $1 LIMIT 1");

        Ok(sqlx::query_as(&query)
            .bind(username)
            .fetch_one(pool)
            .await?)
    }

    /*
       Utilizado somente por admins super user
    */
    /* pub async fn update_otp(pool: &PgPool, id: i64) -> Result<User> {
        let base = &Self::random_base32().to_string();

        let hash = Self::gerar_otp(base);

        Ok(sqlx::query_as!(
            User,
            r#"
            UPDATE auth_user
            SET 
                otp_base32 = $1,
                updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#,
            hash,
            id
        )
        .fetch_one(pool)
        .await?)
    } */

    // Preenche os `None` com os valores atuais do usuário do banco
    /* fn apply_to(current: &User, input: UpdateUser) -> User {
        User {
            id: current.id,
            username: input.username.unwrap_or_else(|| current.username.clone()),
            email: input.email.unwrap_or_else(|| current.email.clone()),
            first_name: input
                .first_name
                .clone()
                .unwrap_or_else(|| current.first_name.clone()),
            last_name: input
                .last_name
                .clone()
                .unwrap_or_else(|| current.last_name.clone()),
            is_active: input.is_active,
            is_staff: input.is_staff,
            is_superuser: input.is_superuser,
            last_login: current.last_login,
            password: current.password.clone(),
        }
    } */
}
