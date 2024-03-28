CREATE VIEW aggregated_tasks AS (
    SELECT
        t.id AS task_id,
        t.deadline_at AS deadline,
        ts.title AS status,
        tr.transitioned_at AS timestamp,
        obj.place_id AS place_id,
        st_latitude(p.location) AS lat,
        st_longitude(p.location) AS lon,
        r.title AS region
    FROM tasks AS t
    JOIN task_transitions tr ON tr.task_id = t.id
    JOIN task_stages ts ON ts.id = tr.task_stage_id
    JOIN service_object_task sot ON sot.task_id = t.id
    JOIN service_objects obj ON obj.id = sot.service_object_id
    JOIN places p ON p.id = obj.place_id
    JOIN regions r ON r.id = p.region_id
);
