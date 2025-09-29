-- Migration: Tabelas do sistema Kanban
-- Criação das tabelas para o dashboard Kanban personalizado por usuário

-- Tabela de Boards (Quadros Kanban)
CREATE TABLE IF NOT EXISTS public.kanban_board (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    color VARCHAR(7) DEFAULT '#3B82F6', -- Cor em hexadecimal
    is_active BOOLEAN DEFAULT true,
    position INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT fk_kanban_board_user FOREIGN KEY (user_id) 
        REFERENCES public.auth_user (id) 
        ON DELETE CASCADE
);

-- Tabela de Colunas do Kanban
CREATE TABLE IF NOT EXISTS public.kanban_column (
    id SERIAL PRIMARY KEY,
    board_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    position INTEGER DEFAULT 0,
    color VARCHAR(7) DEFAULT '#6B7280', -- Cor em hexadecimal
    max_cards INTEGER DEFAULT NULL, -- Limite de cards por coluna (WIP limit)
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT fk_kanban_column_board FOREIGN KEY (board_id) 
        REFERENCES public.kanban_board (id) 
        ON DELETE CASCADE
);

-- Tabela de Cards do Kanban
CREATE TABLE IF NOT EXISTS public.kanban_card (
    id SERIAL PRIMARY KEY,
    column_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    priority VARCHAR(20) DEFAULT 'medium', -- low, medium, high, urgent
    tags TEXT[], -- Array de tags
    color VARCHAR(7) DEFAULT '#F3F4F6', -- Cor em hexadecimal
    position INTEGER DEFAULT 0,
    due_date TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    completed_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
    is_archived BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT fk_kanban_card_column FOREIGN KEY (column_id) 
        REFERENCES public.kanban_column (id) 
        ON DELETE CASCADE,
    CONSTRAINT chk_priority CHECK (priority IN ('low', 'medium', 'high', 'urgent'))
);

-- Tabela de Comentários dos Cards
CREATE TABLE IF NOT EXISTS public.kanban_comment (
    id SERIAL PRIMARY KEY,
    card_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT fk_kanban_comment_card FOREIGN KEY (card_id) 
        REFERENCES public.kanban_card (id) 
        ON DELETE CASCADE,
    CONSTRAINT fk_kanban_comment_user FOREIGN KEY (user_id) 
        REFERENCES public.auth_user (id) 
        ON DELETE CASCADE
);

-- Tabela de Anexos dos Cards
CREATE TABLE IF NOT EXISTS public.kanban_attachment (
    id SERIAL PRIMARY KEY,
    card_id INTEGER NOT NULL,
    filename VARCHAR(255) NOT NULL,
    original_filename VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    file_path TEXT NOT NULL,
    uploaded_by INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT fk_kanban_attachment_card FOREIGN KEY (card_id) 
        REFERENCES public.kanban_card (id) 
        ON DELETE CASCADE,
    CONSTRAINT fk_kanban_attachment_user FOREIGN KEY (uploaded_by) 
        REFERENCES public.auth_user (id) 
        ON DELETE CASCADE
);

-- Índices para performance
CREATE INDEX idx_kanban_board_user_id ON public.kanban_board(user_id);
CREATE INDEX idx_kanban_board_active ON public.kanban_board(is_active);

CREATE INDEX idx_kanban_column_board_id ON public.kanban_column(board_id);
CREATE INDEX idx_kanban_column_position ON public.kanban_column(board_id, position);

CREATE INDEX idx_kanban_card_column_id ON public.kanban_card(column_id);
CREATE INDEX idx_kanban_card_position ON public.kanban_card(column_id, position);
CREATE INDEX idx_kanban_card_priority ON public.kanban_card(priority);
CREATE INDEX idx_kanban_card_due_date ON public.kanban_card(due_date);

CREATE INDEX idx_kanban_comment_card_id ON public.kanban_comment(card_id);
CREATE INDEX idx_kanban_attachment_card_id ON public.kanban_attachment(card_id);

-- Inserir dados iniciais para demonstração
INSERT INTO public.kanban_board (user_id, title, description, color) 
VALUES 
(1, 'Meu Primeiro Quadro', 'Quadro Kanban para organização pessoal', '#3B82F6'),
(1, 'Projetos de Trabalho', 'Acompanhamento de tarefas profissionais', '#10B981');

-- Inserir colunas padrão para o primeiro board
INSERT INTO public.kanban_column (board_id, title, position, color) 
VALUES 
(1, 'A Fazer', 0, '#EF4444'),
(1, 'Em Progresso', 1, '#F59E0B'),
(1, 'Revisão', 2, '#8B5CF6'),
(1, 'Concluído', 3, '#10B981');

-- Inserir alguns cards de exemplo
INSERT INTO public.kanban_card (column_id, title, description, priority, position) 
VALUES 
(1, 'Implementar Dashboard Kanban', 'Criar sistema completo de Kanban para usuários', 'high', 0),
(1, 'Estudar Rust/Axum', 'Aprofundar conhecimentos em desenvolvimento web com Rust', 'medium', 1),
(2, 'Configurar Banco de Dados', 'Criar tabelas e relacionamentos necessários', 'high', 0);
