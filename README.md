# Rust Web 全栈开发教程【代码更新至原视频P16】（非原创，代码来自杨旭老师）

## 内容说明
This is the code of video BV1RP4y1G7KF on bilibili. 

这是B站 BV1RP4y1G7KF 这个视频的代码

原视频地址：[Rust Web 全栈开发教程【完结】](https://www.bilibili.com/video/BV1RP4y1G7KF/)

Original video address: [Rust Web Full Stack Development Tutorial [End]](https://www.bilibili.com/video/BV1RP4y1G7KF/)

## 不同文件夹内容说明, Content description of different folders

### Stage 1

Stage 1 包括了原视频中 P2 - P6 部分的内容，包括基础的 TCP server 的构建和 HTTP request and response 部分的内容，以及之后的 Router 和 Server 还有 Handler 的构建。

Stage 1 includes the content of the P2 - P6, part of the original video, including the construction of the basic TCP server and the content of the HTTP request and response part, and the subsequent construction of Router and Server and handler.

### Stage 2

Stage 2 包括了原视频中 P7 部分的内容，主要是使用 Actic-Web 构建一个基础服务器。

Stage 2 includes the content of the P7 part of the original video. The main focus is to build a base server using Actic-Web.

### Stage 3

Stage 3 包括了原视频中 P8-P10 部分的内容，主要是使用 Actic-Web 构建一个REST api 服务器，内容包括了健康检查，课程的添加，按教师id查询课程信息，按教师id和课程id查询信息，所有数据全部保存在内存中。

Stage 3 includes the content of P8-P10 in the original video. It mainly uses Actic-Web to build a REST api server. The content includes health checks, adding courses, querying course information by teacher id, querying course information by teacher id and course id, all data are stored in memory.

### Stage 4

Stage 4 包括了原视频中 P11 部分的内容，主要是使用 sqlx 查询 yx 数据库中的 Course 表的数据。

Stage 4 includes the content of P11 in the original video, mainly using sqlx to query the data of the Course table in the yx database.

### Stage 5

Stage 5 包括了原视频中 P12-P13 部分的内容，主要是使用 sqlx 查询 yx 数据库中的 Course 表的数据，同时将 Stage 3 中的数据源更换为 postgres 数据库。

Stage 5 includes the content of P12-P13 in the original video. It mainly uses sqlx to query the data of the Course table in the yx database, and replaces the data source in Stage 3 with the postgres database.

### Stage 6

Stage 6 包括了原视频中 P14-P15 部分的内容，主要内容是添加错误处理的部分。

Stage 6 includes the content of P14-P15 in the original video, and the main content is to add error handling.

### Stage 7

Stage 7 包括了原视频中 P16 部分的内容，主要内容是完善已有的服务器。该部分内容较多，对应的原视频时长也较长，其中也存在 PostgreSQL DB 中的一些知识，需要特别注意。

Stage 7 includes the content of P16 in the original video, the main content is to improve the existing server. This part has a lot of content, and the corresponding original video is also longer, and there is also some knowledge in the PostgreSQL DB, which requires special attention.

### Stage 8

Stage 8 原计划编写原视频中 P17 的内容，但是看完后发现内容只是 P16 的重复，写会 Stage 7 的情况下，完全不存在任何难度，故略去。

Stage 8 originally planned to write the content of P17 in the original video, but after watching it, I found that the content is just a repetition of P16. In the case of writing Stage 7, there is no difficulty at all, so it is omitted.

### Stage 9

之后的内容更多的偏重于 WebAssembly，我不感兴趣，所以不写了。

The content after that is more focused on WebAssembly, I am not interested, so I will not write it.


## 文件夹树，File tree

```shell
├─stage_1
│  ├─http
│  │  └─src
│  ├─httpserver
│  │  ├─data
│  │  ├─public
│  │  └─src
│  ├─tcpclient
│  │  └─src
│  └─tcpserver
│      └─src
├─stage_2
│  └─src
│      └─bin
├─stage_3
│  └─src
│      └─bin
├─stage_4
│  └─src
├─stage_5
│  └─src
│      └─bin
├─stage_6
│  └─src
│      └─bin
└─stage_7
    └─src
        ├─bin
        ├─db_access
        ├─handlers
        └─models
```


## Rust 版本, Rust verion

```shell
>> rustup --version

rustup 1.25.1 (bb60b1e89 2022-07-12)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.65.0 (897e37553 2022-11-02)`
```


## 推荐在学习中使用的插件（按字母顺序排序）
1. CodeLLDB
2. Even Better TOML
3. Git History
4. GitLens
5. rust-analyzer



## 利益相关说明

该 Repo 旨在帮助懒得打字的朋友们快速提供一份能拿来跑的代码，不用于任何盈利目的，也不存在任何形式的捐赠等内容，仅为人肉 OCR 的一份存档

This Repo aims to help friends who are not willing to type, just quickly provide a code that can be used to run. It is not used for any profit purpose, and there is no donation in any form. It is only an archive of OCR by human's eye.
