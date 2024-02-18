# Jabu
A build system as well as an automation tool written in Rust for Java project and dependency management.

## 1. How to compile and install from sources

```bash
git clone https://github.com/folgue02/jabu.git
cd jabu
cargo install --path .
```

## 2. User guide

### 2.1 Basic concepts

Jabu's functionality is divided in subcommands (*such as `build` or `run`*) which are called *'tasks'*. These tasks represent actions to be performed, they can be categorized in two types of tasks:

- ___Jabu tasks___, which have to be executed on a jabu project (*i.e. the `run` task can only be executed on a project, since its purpose is to run a project*).
- ___Normal tasks/tasks___, these can be executed anywhere, no need to be in a project (*i.e. `new` would be one, since its in charge of creating a new project*).

### 2.2 Project generation

To create a jabu project, jabu has a built-in task named `new`, which can generate a project with a given name, and of a given type.

```bash
jabu new --name:myProject --project-type:bin
```

The `--name` flag specifies the name of the project (*this one is required*), while the `--project-type` flag species which type of project it will be, therefore, also changing the structure of
the project when generating it (*this flag is not required, and by default it will refer to `bin`, an executable project*).

### 2.3 Generated project's structure

This last command has generated an structure, let's look at it.

```
[folgue ->(0) ~/source/misc/myProject]$ tree .
.
├── jabu.ron       # project's configuration
├── lib            # Project's dependencies 
├── scripts        # Scripts specific to the project.
├── src            
│   ├── main       # Contains the java sources (no nested 'java' directory)
│   ├── resources  # Contains the application's resources
│   └── test       # Contains the application's test sources
└── target         # Generated objects and files by jabu (classes, jars...)

8 directories, 1 file
```

### 2.4 Hello World

To write our first code in a jabu project, let's create a package under the `src/main` directory, for example, `src/main/com/example/myProject`, and inside of it, let's create 
our Java source file (`src/main/com/example/myProject/App.java`).

```java
package com.example.myProject;

public class App {
    public static void main(String[] args) {
        System.out.println("Hello World from Jabu!");
    }
}
```

To build our project we can use the `build` task like this:

```bash
jabu build
```

This will generate the class files based on the source files we've created. Even if we can execute it manually using the `java` command from our terminal, specifying the classpath and the main class, 
we should use the already provided `run` task.

```
[folgue ->(0) ~/source/misc/myProject]$ jabu run
Java installation detected: "/usr/lib/jvm/default"
=> Executing dependency task 'build' with args '[]'
Sources to compile: 

==> [CMD]: ["/usr/lib/jvm/default/bin/javac" ["--source", "17", "--target", "17", "-d", "./target/classes"]]
error: no source files
Failure:
While executing a task there was an error executing its dependency task 'build': Command 'javac' with the following error/error code: 2
```

Something went wrong, the sources have been compiled, yet we are not allowed to execute our project. This is because we didn't tell jabu what main class to execute, which can be solved
in two different ways.

- ___Using the `--main-class:...` flag___, which is an optional flag that allows us to tell jabu directly what class to be executed.
- ___Specifying it in the `jabu.ron` configuration file___, which not only tells jabu during this task which class to execute, but marks the project as executable, as well marking the specified class as the project's main class.

#### 2.4.1 First method

```
[folgue ->(0) ~/source/misc/myProject]$ jabu run --main-class:com.example.myProject.App
Java installation detected: "/usr/lib/jvm/default"
=> Executing dependency task 'build' with args '[]'
Sources to compile: 
1: "./src/main/com/example/myProject/App.java"

==> [CMD]: ["/usr/lib/jvm/default/bin/javac" ["./src/main/com/example/myProject/App.java", "--source", "17", "--target", "17", "-d", "./target/classes"]]
==> [CMD]: ["/usr/lib/jvm/default/bin/java" ["-cp", "./target/classes", "com.example.myProject.App"]]
Hello World from jabu!
```

The last line of the output is our code's output.

#### 2.4.2 Second method (modifying the project's configuration)

First, we'll open the project's configuration file, `jabu.ron`, which we will discuss in next chapters.

```ron
(
    header: (
        project_name: "myProject",
        author: "anon",
        description: "A Java project.",
        version: "0.0.1",
    ),
    java_config: (
        java_version: 17,
        source: 17,
        target: 17,
    ),
    fs_schema: (
        source: "./src/main",
        target: "./target",
        lib: "./lib",
        resources: "./src/resources",
        scripts: "./scripts/",
        test: "./src/test",
        other: [],
    ),
    properties: {}, // <- We'll only focus on this part
)
```

The properties field contains properties that will be used to generate the Manifest file for our project, but the only thing we have 
to know is that we can specify the project's main class by creating a key with the name `Main-Class` and the desired main class as a value, just like this:

```ron
// ...
    properties: {
        "Main-Class": "com.example.myProject.App"
    }
// ...
```

Now we can finally execute our code.

```
Java installation detected: "/usr/lib/jvm/default"
=> Executing dependency task 'build' with args '[]'
Sources to compile: 
1: "./src/main/com/example/myProject/App.java"

==> [CMD]: ["/usr/lib/jvm/default/bin/javac" ["./src/main/com/example/myProject/App.java", "--source", "17", "--target", "17", "-d", "./target/classes"]]
==> [CMD]: ["/usr/lib/jvm/default/bin/java" ["-cp", "./target/classes", "com.example.myProject.App"]]
Hello World from jabu!
```

(*The last line is the output of our application*)

### 2.5 The `jabu.ron` file

Every jabu project contains a configuration file with the name `jabu.ron`, so let's take a deeper look at our project's configuration file:

```ron
(
    header: (                           // 1.
        project_name: "myProject",
        author: "anon",                 
        description: "A Java project.",
        version: "0.0.1",
    ),
    java_config: (                      // 2.
        java_version: 17,
        source: 17,
        target: 17,
    ),
    fs_schema: (                        // 3.
        source: "./src/main",
        target: "./target",
        lib: "./lib",
        resources: "./src/resources",
        scripts: "./scripts/",
        test: "./src/test",
        other: [],
    ),
    properties: {                       // 4.
        "Main-Class": "com.example.myProject.App"
    },
)
```

1. The project's header, this contains the main information about the project, being

    - `project_name`, the project's name.
    - `author`, the project's author (*which can also be the groupId in other build systems like maven or gradle*).
    - `description`, a brief description about the project.
    - `version`, the project's current version.

2. The java configuration of the project, which specifies with which jdk standard it should compile, and its compatibility.
3. This part of the configuration shouldn't be modified by the user, as it's just created by the `new` task to tell jabu where each directory is and 
it usually stays the same for the rest of the life of the application.
4. Properties of the project, which will be passed as the project's manifest file.
