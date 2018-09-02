## HelioDB todo list.
### Stage 1
 - [x] Table definition query
    - [x] Base query
    - [x] Field types

### Stage 2
 - [x] Data selection query
    - [x] Single table
    - [x] Base query
    - [x] Specific fields

### Stage 3
 - [ ] Data modification query
     - [ ] Insert
     - [ ] Update
     - [ ] Delete

### Stage 4
 - [ ] Data selection query
    - [ ] Multiple tables

### Stage 5
 - [ ] Table modification query
    - [ ] Specific fields

### Stage 6
 - [ ] Table definition query
    - [ ] Data storing according to table definition

### Stage 7
Database entity
 - [ ] Allow multiple databases
 - [ ] Add user management
 - [ ] Add user authentication

### Stage 8
 - [ ] Reference to another table field type.

### Stage 9
 - [ ] Multiple queries per request
 ```
 [{<query>}, {<query>}, {<query>}].
 ```

### Stage 10
 - [ ] Transactions?

## Glossaries
 Storage  - table, that has definition and data that match this definition

 Database - collection of storages

## Examples

Selection :
```json
{
    "select" : {
        "storage" : "<storage name>",
        "fields"  : ["<fields-1>", "<fields-2>"]
    }
}
```

Definition:
```json
{
    "define" : {
        "storage" : "<storage name>",
        "fields"  : {
            "<field name>" : "<integer|string|float>"
            ...
        }
    }
}
```

Insertion:
```json
{
    "insert" : {
        "storage" : "<storage name>",
        "data" : [
            {"<field-1>": "<value>", "<field-2>": "<value>"},
            {"<field-1>": "<value>", "<field-2>": "<value>"},
        ]
    }
}
```