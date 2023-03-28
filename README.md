# unwrap-to-question-mark-challenge

## The Challenge
The challenge in this project is to refactor this Rust code that uses "unwrap" to instead use the question mark operator. Feel free to do any additional refactorings as well!

<br/>

## Why?
The `unwrap` makes any error immediately crash your program and unwind. While this can be nice for moving quickly during development, panicking will crash every thread and makes your function signatures less descriptive. For this reason, many Rustaceans recommend refactoring `unwrap` to `?` before considering the code "production ready".

Changing from unwrap to ? can be tricky though, especially in situations (like this one) where a function could throw different kinds of errors. The hypothesis is that by solving this relatively simple unwrap-to-? you learn how to go from unwrap to ? in general!

## How To Submit
Open a PR with your refactorings to submit your solution!
