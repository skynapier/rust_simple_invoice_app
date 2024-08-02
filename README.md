### requirements?
* You can modify any of the code in this solution, split out classes etc
* You can modify Invoice and InvoiceLine, rename and add methods, change property types (hint) 
* Feel free to use any libraries or frameworks you like
* Feel free to write tests (hint) 


### benefit of rust
* no empty null case dont need worry about some edge cases (null pointer exception)
* data racing (async if in future)
* zero cost abstraction (if large/big invoice lines)
* argument passing (pass by value if add? ownership? refrence?)
* utf8 support (invoice content in other language)


### problems?

1. invoice number? item id? unique?
2. remove by id or index?
3. merge? date? id?
4. returning method on service? integration test?


### how to run
* ```docker-compose build docker-compose up```
* ```cargo build cargo run cargo test```

```shell
cargo fmt
```

### what can be update?
* base on the require, web server? processor? cli tool?
* presist data? caching table for UUID?
* test cases, using cucumber for behaviour explaination?
* mocking test? for inject different invoice to services
* splitting into different project and dependency injection? 
* CICD pipeline? trigger by push