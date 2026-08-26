-- Add migration script here
CREATE TABLE qa_runs(
 run_id UUID PRIMARY KEY,
 project_id UUID NOT NULL 
    REFERENCES projects(project_id)
    ON DELETE CASCADE,

 target_url TEXT NOT NULL,
 figma_file_key TEXT NOT NULL,
 figma_node_key TEXT NOT NULL,

 status TEXT NOT NULL DEFAULT 'pending'
    CHECK(STATUS IN ('pending', 'running', 'completed', 'failed')),

 created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
 started_at TIMESTAMPTZ,
 completed_at TIMESTAMPTZ              

);