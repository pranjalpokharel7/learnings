
![[Pasted image 20240622205300.png]]
- libpq is used to communicate between client and server (for read() and write() syscalls basically). DBeaver used the jdbc driver for the same purpose. Fun fact about libpq: when it was originally developed, the TCP/IP protocol was not the standard as it is today, so it re-implements a lot of things that TCP/IP handles today.
- The initial connection and auth is done with the postmaster which is responsible for spawning server processes (fork()). A new process is spawned for each db connection.
- The shared memory contains tables and disk buffers that are shared between postgres server processes. The connection pool only see each other through the shared memory.
- There are "auxiliary" processes responsible for db maintenance. Server processes are basically a combination of the postgres backend and these auxiliary processes.

## Disk Layout

![[Pasted image 20240622210220.png]]
- Cluster means multiple dbs within the same instance (every other create database command adds to this cluster).
- The dbs in a cluster are isolated from each other and you will need to use the `dblink` command to execute statements between them (my terminology might be wrong here).
- However, you can have multiple schemas within the same db and you can perform joins between tables of different schemas (can even have foreign keys).
- Says something about how things are structured internally, which I'm skipping for now.

## "Backend" Process Flow

- Backend here is the process inside UNIX that handles your connection.
![[Pasted image 20240622211205.png]]
Presentation on the backend flowchart by Bruce Momijan, the fucking GOD (will look for the original presentation link, screenshotted for now). The green outline is the backend process flow. Main concept is the parser -> optimizer -> executor flow.

### libpq

- Since libpq is a custom protocol, it needs to do handle the bulk of the network operations, even if it runs today over TCP/IP. Some tasks include handling network timeouts, handle SSL certificates, and more.
- Supports asynchronous notifications through LISTEN/NOTIFY (https://www.postgresql.org/docs/current/libpq-notify.html)
- Also provides the COPY command for bulk data transfer (https://www.postgresql.org/docs/current/libpq-copy.html) - insert large number of rows at once (for eg, from CSV files)


### parser

#### lexical analysis
- Lexical analysis using flex and bison (tools for lexical analysis - https://www.oreilly.com/library/view/flex-bison/9780596805418/ch01.html - what a fucking interesting chapter, will definitely revisit this).
- For eg, we need to map the SELECT statement to a corresponding C function in the backend, which is expressed through parsed tokens.

#### parse analysis
- Check if table exists, column exists and so on.