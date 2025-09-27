-- Add migration script here
CREATE TABLE IF NOT EXISTS public.cadastro_uf
(
    id integer NOT NULL DEFAULT nextval('cadastro_uf_id_seq'::regclass),
    sigla character varying(2) COLLATE pg_catalog."default" NOT NULL,
    nome character varying(70) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT cadastro_uf_pkey PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.cadastro_municipio
(
    id integer NOT NULL DEFAULT nextval('cadastro_municipio_id_seq'::regclass),
    uf_id integer NOT NULL,
    nome character varying(70) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT cadastro_municipio_pkey PRIMARY KEY (id),
    CONSTRAINT cadastro_municipio_uf_id_fkey FOREIGN KEY (uf_id)
        REFERENCES public.cadastro_uf (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        DEFERRABLE INITIALLY DEFERRED
);

CREATE TABLE IF NOT EXISTS public.cadastro_folha
(
    id bigint NOT NULL DEFAULT nextval('cadastro_folha_id_seq'::regclass),
    orgao_id integer NOT NULL,
    ano integer NOT NULL,
    mes integer NOT NULL,
    servidor_id integer NOT NULL,
    salario numeric(15,2) NOT NULL,
    base_fgts numeric(15,2) NOT NULL,
    base_inss numeric(15,2) NOT NULL,
    base_irrf numeric(15,2) NOT NULL,
    ded_irrf numeric(15,2) NOT NULL,
    cargo_id integer NOT NULL,
    setor_id integer NOT NULL,
    departamento_id integer NOT NULL,
    vinculo_id integer NOT NULL,
    CONSTRAINT cadastro_folha_pkey PRIMARY KEY (id),
    CONSTRAINT cadastro_folha_cargo_id_fkey FOREIGN KEY (cargo_id)
        REFERENCES public.cadastro_cargo (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        DEFERRABLE INITIALLY DEFERRED,
    CONSTRAINT cadastro_folha_departamento_id_fkey FOREIGN KEY (departamento_id)
        REFERENCES public.cadastro_departamento (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        DEFERRABLE INITIALLY DEFERRED,
    CONSTRAINT cadastro_folha_orgao_id_fkey FOREIGN KEY (orgao_id)
        REFERENCES public.cadastro_orgao (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        DEFERRABLE INITIALLY DEFERRED,
    CONSTRAINT cadastro_folha_servidor_id_fkey FOREIGN KEY (servidor_id)
        REFERENCES public.cadastro_servidor (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        DEFERRABLE INITIALLY DEFERRED,
    CONSTRAINT cadastro_folha_setor_id_fkey FOREIGN KEY (setor_id)
        REFERENCES public.cadastro_setor (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        DEFERRABLE INITIALLY DEFERRED,
    CONSTRAINT cadastro_folha_vinculo_id_fkey FOREIGN KEY (vinculo_id)
        REFERENCES public.cadastro_tipovinculo (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        DEFERRABLE INITIALLY DEFERRED
);


