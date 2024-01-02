# Managing growing projects with Packages, Crates and Modules

As a project grows, you should organize code by splitting it into multiple modules and then multiple files.

By grouping related functionality and separating code with distinct features, you’ll clarify where to find code that implements a particular feature and where to go to change how a feature works.

The way you write code defines which parts are public for other code to use and which parts are private implementation details that you reserve the right to change.

When reading, writing, and compiling code, programmers and compilers need to know whether a particular name at a particular spot refers to a variable, function, struct, enum, module, constant, or other item and what that item means.
The nested context in which code is written has a set of names that are defined as “in scope. You can create scopes and change which names are in or out of scope. You can’t have two items with the same name in the same scope; tools are available to resolve name conflicts.

## the module system

collectively referred to as the module system are a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs:

1. Packages: A Cargo feature that lets you build, test, and share crates
2. Crates: A tree of modules that produces a library or executable
3. Modules and use: Let you control the organization, scope, and privacy of paths
4. Paths: A way of naming an item, such as a struct, function, or module
