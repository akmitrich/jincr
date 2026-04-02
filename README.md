# Jincr

## Basic data scheme

Every row in the document table is an incremental operation: add, delete, replace or (for efficiency) snapshot.

### Table "json_document"

| Column    | Type                                    |
| --------- | --------------------------------------- |
| kind      | ENUM ('snap','replace','delete', 'add') |
| path      | TEXT                                    |
| value     | JSONB                                   |
| timestamp | timestamp with time zone                |
| info      | TEXT                                    |

### Snapshot

The `value` must be non-NULL. `path` is expected to be NULL and ignored.

### Replace

There must be data in the `path`. This data is replaced with `value`.

If `path` leads to nothing operation is ignored.

### Delete

Deletes data in the `path` if there are some. `value` is expected to be NULL and ignored.

### Add

Creates `value` in the `path`. If `path` leads to some data they are replaced with `value`.

### Info

Is any kind of information about operation.
