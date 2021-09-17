pg_stat_statements

https://www.modb.pro/db/40480



PostgreSQL中高耗SQL的获取可以使用pg_stat_statements模块来获取，pg_stat_statements模块提供执行SQL语句的执行统计信息。

该模块必须在postgresql.conf的shared_preload_libraries中增加pg_stat_statements来载入，因为它需要额外的共享内存。增加或移除该模块需要将数据库重启。

当pg_stat_statements被载入时，它会跟踪该服务器的所有数据库的统计信息。该模块提供了视图pg_stat_statements以及函数pg_stat_statements_reset用于访问和操纵这些统计信息。这些视图和函数不是全局可用的，但是可以在指定数据库创建该扩展。

需要数据库进行配置

# 1. 创建扩展模块
创建extension模块
```sql
postgres=# CREATE EXTENSION pg_stat_statements;
```
CREATE EXTENSION修改配置文件

 
# 2. 配置postgresql.conf参数文件
修改数据库PG_HOME下的postgresql.conf文件

```conf
shared_preload_libraries= 'pg_stat_statements'

pg_stat_statements.max= 10000 # pg_stat_statements中记录的最大的SQL条目数，默认为5000，通过LRU算法，覆盖老的记录。

pg_stat_statements.track= all # all - (所有SQL包括函数内嵌套的SQL), top - 直接执行的SQL(函数内的sql不被跟踪), none - (不跟踪)

pg_stat_satements.save = on   # 用来控制数据库在关闭的时候，是否将SQL信息保存到文件中。默认打开

pg_stat_satements.track_utility = on # 是否跟踪非DML语句 (例如DDL，DCL)，on表示跟踪, off表示不跟踪  

track_io_timing = on #如果要跟踪IO消耗的时间，需要打开如上参数

track_activity_query_size = 2048 #设置单条SQL的最长长度，超过被截断显示（可选）

```

如果没有配置postgresql.conf文件中的shared_preload_libraries,那么将会提示如下报错：

ERROR:pg_stat_statements must be loaded via shared_preload_libraries

# 3. 重启数据库
使用pg_ctl重新启动数据库，使扩展生效。

```shell
pg_ctl start -D $PGDATA -l tmp/pg_rotate_logfile()
```
或
```shell
pg_ctl restart -m fast  
```


6. pg_stat_statements使用
log_min_duration_statement这个参数可以控制阈值的时间，如果查询花费的时间长于此阈值时间，则会记录该SQL。默认为1s。可以使用

```sql
ALTER SYSTEM SET log_min_duration_statement = 1000;
```

更改阈值记录，单位为ms

6.1、SQL执行时间获取
我们可以在数据库中看到平均运行时间最高的查询，如下所示：

```sql
SELECT total_time, min_time,(total_time/calls) as avg_time, max_time, mean_time, calls, rows,query

FROM pg_stat_statements

ORDER BY mean_time DESC

LIMIT 10;
```

其中各项的涵义：

total_time：返回查询的总运行时间（以毫秒为单位）。

min_time、avg_time和max_time：返回查询的最小、平均和最大运行时间。

mean_time：使用total_time/调用返回查询的平均运行时间（以毫秒为单位）。

Calls (调用)：返回查询运行的总数。

Rows(行数)：返回由于查询而返回或受影响的行总数。

Query(查询)：返回正在运行的查询。默认情况下，最多显示1024个查询字节。可以使用track_activity_query_size参数更改此值。

# 7、重置pg_stat_statements统计信息
pg_stat_statements所获得的统计数据一直累积到重置。

可以使用以下脚本进行按天备份。

备份完成后可以通过具有超级用户权限的用户连接到数据库以重置统计数据来运行重置：
```sql
SELECT pg_stat_statements_reset();
```


# 常用sql

常用的统计sql参考

最耗IO SQL，单次调用最耗IO SQL TOP 5

select userid::regrole, dbid, query from pg_stat_statements order by (blk_read_time+blk_write_time)/calls desc limit 5;  

 

总最耗IO SQL TOP 5

select userid::regrole, dbid, query from pg_stat_statements order by (blk_read_time+blk_write_time) desc limit 5;  

 

最耗时 SQL，单次调用最耗时 SQL TOP 5

select userid::regrole, dbid, query from pg_stat_statements order by mean_time desc limit 5;  

 

总最耗时 SQL TOP 5

select userid::regrole, dbid, query from pg_stat_statements order by total_time desc limit 5;  

 

响应时间抖动最严重 SQL

select userid::regrole, dbid, query from pg_stat_statements order by stddev_time desc limit 5;  

 

最耗共享内存 SQL

select userid::regrole, dbid, query from pg_stat_statements order by (shared_blks_hit+shared_blks_dirtied) desc limit 5;  

 

最耗临时空间 SQL

select userid::regrole, dbid, query from pg_stat_statements order by temp_blks_written desc limit 5;  

# 注意
One thing to keep in mind is that the query texts are "kept in an external disk file, and do not consume shared memory" (taken from the official docs). pg_stat_statements should leave only a relatively small footprint on your system especially compared to logging all of the things. That said, you could also make sure to set a lower threshold on pg_stat_statements.max, or set only certain types of statements to be tracked using the pg_stat_statements.track parameters.


# 本地测试

命令行切换到postgres用户
```shell
createdb bench
pgbench -i bench
pgbench -c10 -t300 bench
```

在psql客户端执行
```sql
postgres=# SELECT query, calls, total_time, rows, 100.0 * shared_blks_hit /
                nullif(shared_blks_hit + shared_blks_read, 0) AS hit_percent
          FROM pg_stat_statements ORDER BY total_time DESC LIMIT 5;
                                query                                | calls |    total_time     | rows |     hit_percent      
---------------------------------------------------------------------+-------+-------------------+------+----------------------
 UPDATE pgbench_branches SET bbalance = bbalance + $1 WHERE bid = $2 |  3000 | 360203.4763000002 | 3000 | 100.0000000000000000
 UPDATE pgbench_tellers SET tbalance = tbalance + $1 WHERE tid = $2  |  3000 | 293166.3799000001 | 3000 | 100.0000000000000000
 CREATE DATABASE bench                                               |     1 |         1112.3739 |    0 |  81.2500000000000000
 UPDATE pgbench_accounts SET abalance = abalance + $1 WHERE aid = $2 |  3000 | 737.0728000000007 | 3000 |  98.5808648983383218
 alter table pgbench_accounts add primary key (aid)                  |     1 |          116.9855 |    0 | 100.0000000000000000
 ```
```sql

postgres=# SELECT total_time, min_time,(total_time/calls) as avg_time, max_time, mean_time, calls, rows,query
FROM pg_stat_statements
ORDER BY mean_time 
LIMIT 5;
    total_time     | min_time |       avg_time        | max_time |       mean_time       | calls | rows |                              query                              
-------------------+----------+-----------------------+----------+-----------------------+-------+------+-----------------------------------------------------------------
            0.0008 |   0.0008 |                0.0008 |   0.0008 |                0.0008 |     1 |    0 | commit
             0.001 |    0.001 |                 0.001 |    0.001 |                 0.001 |     1 |    0 | begin
 5.319000000000038 |   0.0003 | 0.0017730000000000126 |    0.111 | 0.0017729999999999977 |  3000 |    0 | END
 5.652200000000043 |   0.0003 |  0.001884066666666681 |   0.0608 | 0.0018840666666666652 |  3000 |    0 | BEGIN
            0.0689 |   0.0042 |               0.00689 |   0.0299 |               0.00689 |    10 |   10 | insert into pgbench_tellers(tid,bid,tbalance) values ($1,$2,$3)
(5 rows)

```

```sql
SELECT total_time, min_time,(total_time/calls) as avg_time, max_time, mean_time, calls, rows,query FROM pg_stat_statements ORDER BY rows desc limit 5 ;
    total_time     |       min_time       |       avg_time       | max_time  |      mean_time       | calls |  rows  |                                                query                                                 
-------------------+----------------------+----------------------+-----------+----------------------+-------+--------+------------------------------------------------------------------------------------------------------
           52.1607 |              52.1607 |              52.1607 |   52.1607 |              52.1607 |     1 | 100000 | copy pgbench_accounts from stdin
 80.93629999999992 | 0.005699999999999999 |  0.02697876666666664 |    0.3944 |  0.02697876666666666 |  3000 |   3000 | SELECT abalance FROM pgbench_accounts WHERE aid = $1
 737.0728000000007 | 0.016399999999999998 |  0.24569093333333358 |  258.1977 |   0.2456909333333331 |  3000 |   3000 | UPDATE pgbench_accounts SET abalance = abalance + $1 WHERE aid = $2
 293166.3799000001 |               0.0098 |    97.72212663333337 | 1395.9803 |    97.72212663333313 |  3000 |   3000 | UPDATE pgbench_tellers SET tbalance = tbalance + $1 WHERE tid = $2
 68.91720000000012 | 0.005699999999999999 | 0.022972400000000042 |    0.3077 | 0.022972399999999997 |  3000 |   3000 | INSERT INTO pgbench_history (tid, bid, aid, delta, mtime) VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
(5 rows)
```


```sql
postgres=# select queryid,calls, rows,shared_blks_hit,shared_blks_read,shared_blks_written,local_blks_hit,local_blks_dirtied,local_blks_written from pg_stat_statements ;
       queryid        | calls | rows | shared_blks_hit | shared_blks_read | shared_blks_written | local_blks_hit | local_blks_dirtied | local_blks_written 
----------------------+-------+------+-----------------+------------------+---------------------+----------------+--------------------+--------------------
 -8617639074358654804 |     1 |    0 |              43 |                0 |                   0 |              0 |                  0 |                  0
  1014626868305795872 |     1 |    1 |               4 |                0 |                   0 |              0 |                  0 |                  0
  1809546128015582813 |     2 |   40 |              44 |                0 |                   0 |              0 |                  0 |                  0
  2397681704071010949 |  3000 |    0 |               0 |                0 |                   0 |              0 |                  0 |                  0
  2634098840000013830 |     2 |    2 |              17 |                0 |                   0 |              0 |                  0 |                  0
 -7810315603562552972 |  3000 |    0 |               0 |                0 |                   0 |              0 |                  0 |                  0
    31467593155865701 |  3000 | 3000 |            9085 |                0 |                   0 |              0 |                  0 |                  0
  5221567155341688057 |     1 |    0 |              41 |                0 |                   0 |              0 |                  0 |                  0
  8181408706099434098 |  3000 | 3000 |           12474 |                0 |                  10 |              0 |                  0 |                  0
   274605727702177504 |     2 |   43 |               0 |                0 |                   0 |              0 |                  0 |                  0
 -2957143801391021615 |     2 |    2 |               8 |                0 |                   0 |              0 |                  0 |                  0
  4936189456064391544 |     2 |   45 |              59 |                0 |                   0 |              0 |                  0 |                  0
 -2348822008752871029 |     1 |   19 |               0 |                0 |                   0 |              0 |                  0 |                  0
  6878671420186503522 |     2 |   37 |               0 |                0 |                   0 |              0 |                  0 |                  0
  -659047027458955830 |     1 |    0 |              71 |                0 |                   0 |              0 |                  0 |                  0
  8404882884439976922 |  3000 | 3000 |           32662 |                0 |                   0 |              0 |                  0 |                  0
  1724937038400323112 |     1 |   20 |               0 |                0 |                   0 |              0 |                  0 |                  0
 -5873253136260474870 |     6 |   11 |               0 |                0 |                   0 |              0 |                  0 |                  0
 -8495638298558093625 |  3000 | 3000 |            3065 |                2 |                  30 |              0 |                  0 |                  0
  -614501100270111550 |     2 |   46 |             109 |                0 |                   0 |              0 |                  0 |                  0
  1479389428151783715 |     1 |    1 |               0 |                0 |                   0 |              0 |                  0 |                  0
  2079142303721575618 |  3000 | 3000 |           49801 |                0 |                   0 |              0 |                  0 |                  0

```


# 7、重置pg_stat_statements统计信息

```sql
SELECT pg_stat_statements_reset();
```
可以看到已经找不到之前的统计记录
```sql
postgres=# SELECT total_time, min_time,(total_time/calls) as avg_time, max_time, mean_time, calls, rows,query FROM pg_stat_statements ORDER BY rows desc limit 5 ;
 total_time | min_time | avg_time | max_time | mean_time | calls | rows |               query               
------------+----------+----------+----------+-----------+-------+------+-----------------------------------
     0.0789 |   0.0789 |   0.0789 |   0.0789 |    0.0789 |     1 |    1 | SELECT pg_stat_statements_reset()
(1 row)
```


log_min_duration_statement 这个参数可以控制阈值的时间，如果查询花费的时间长于此阈值时间，则会记录该SQL。默认为1s。可以使用
```sql
ALTER SYSTEM SET log_min_duration_statement = 1000;
```
> 实际测试中并没有生效

然后重新执行测试命令
```shell
pgbench -c10 -t300 bench
```

```sql

```
