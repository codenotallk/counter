# MVC Simple Counter

In this project I want to learn how to use MVC in Rust.

In the diagram I'll implement the Counter Service first.

Here I'll need to add a serializer here. I'll use serde for that.

In rust all these things are easy to accomplish. Bye

We finished the Counter service. Let's mark in the diagram.

Now I'll implement the display service.

It will have two threads. One for controller which will receive the messages
in json enconding. The thread two will show the value on the screen.

Rust makes things a little complicate. In this version I won't use repository.
In the next version I'll create a new branch to implement the repository

This version I made through channel. that's it.

For this version I'll create a repository. But first, I need to create another branch 
to have both solutions.

Now we have a solution using repository. But it is a little verbose. Sharing references in rust needs some special threatment. That's it . Now we can make as complete all the diagram. Nice. Bye