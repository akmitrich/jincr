# Jincr

## Basic data scheme
Every row in the document table is an incremental operation: add, delete or (for efficiency) snapshot.

<h1>Table "json_document"</h1>

| Column    | Type                     |
| --------- | ------------------------ |
| path      | TEXT                     |
| value     | JSONB                    |
| timestamp | timestamp with time zone |
| info      | TEXT                     |

### Path is NULL
The `value` must be non-NULL and this is snapshot

### Path is not NULL
- if `value` is NULL delete the whole `path`
- if `value` is not NULL add the `value` on `path`

### Info
Is any kind of information about operation.