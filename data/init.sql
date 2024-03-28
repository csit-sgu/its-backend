CREATE VIEW aggregated_tasks AS (
    SELECT
        t.id AS task_id,
        t.taskable_type AS task_type,
        t.deadline_at AS task_deadline,
        t.account_id AS task_account_id,
        t.created_at AS task_created_at,
        ts.title AS task_transition_title,
        ts.is_start AS task_stage_is_start,
        ts.is_fulfilled AS task_stage_is_fulfilled,
        ts.is_closed AS task_stage_is_closed,
        ts.is_canceled AS task_stage_is_cancelled,
        tr.transitioned_at AS task_transitioned_at,
        obj.id AS object_id,
        obj.place_id AS object_place_id,
        st_latitude(p.location) AS place_lat,
        st_longitude(p.location) AS place_lon,
        r.id AS region_id,
        r.title AS region_title
    FROM tasks AS t
    JOIN task_transitions tr ON tr.task_id = t.id
    JOIN task_stages ts ON ts.id = tr.task_stage_id
    JOIN service_object_task sot ON sot.task_id = t.id
    JOIN service_objects obj ON obj.id = sot.service_object_id
    JOIN places p ON p.id = obj.place_id
    JOIN regions r ON r.id = p.region_id
);

CREATE VIEW transition_view AS (
    SELECT tt.id, tt.task_id, u.name as transitioned_by, tt.transitioned_at, ts.title as stage_title
    FROM task_transitions as tt
    INNER JOIN task_stages as ts ON tt.task_stage_id = ts.id
    INNER JOIN users as u on tt.transitioned_by = u.id
    ORDER BY tt.transitioned_at
);
