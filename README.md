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
     - [x] Insert
     - [ ] Update
     - [ ] Delete

### Refactoring
 - [ ] Restructure
 - [ ] Simplify
 - [ ] Docs

### Stage 4
 - [ ] Data modification queries transaction
     - [ ] Insert only if all is valid
     - [ ] Update only if all is valid
     - [ ] Delete only if all is valid

### Stage 5
 - [ ] Data selection query
    - [ ] Multiple tables

### Stage 6
 - [ ] Table modification query
    - [ ] Specific fields

### Refactoring
 - [ ] Restructure
 - [ ] Simplify
 - [ ] Docs

### Stage 7
 - [ ] Table definition query
    - [ ] Data storing according to table definition

### Stage 8
Database entity
 - [ ] Allow multiple databases
 - [ ] Add user management
 - [ ] Add user authentication

### Stage 9
 - [ ] Field nullability
 - [ ] Reference to another table field type.

### Refactoring
 - [ ] Restructure
 - [ ] Simplify
 - [ ] Docs

### Stage 10
 - [ ] Multiple queries per request
 ```
 [{<query>}, {<query>}, {<query>}].
 ```

### Stage 11
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