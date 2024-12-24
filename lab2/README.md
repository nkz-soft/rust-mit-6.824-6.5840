### [6.5840 Lab 2: Key/Value Server](https://pdos.csail.mit.edu/6.824/labs/lab-kvsrv.html)

In this lab you will build a key/value server for a single machine that ensures that each operation is executed exactly
once despite network failures and that the operations are linearizable. 

### 1. Preliminary preparation

1. LAB is officially implemented based on go, but you need to understand the details of some code comments,
   the overall framework and the test situation, so you still need to download and read it carefully.
    ```bash
    git clone git://g.csail.mit.edu/6.5840-golabs-2024
    ```