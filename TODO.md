### 1. **String（字符串）**
- **存储指令**：`SET <key> <value>`
    - 示例：`SET name "John Doe"`
- **获取指令**：`GET <key>`
    - 示例：`GET name` → 返回 `"John Doe"`
- // TODO 再加一个del指令

---

### 2. **Hash（哈希）**
- **存储指令**：`HSET <key> <field> <value>`
    - 示例：`HSET user:1 name "John" age 30`
- **获取单个字段**：`HGET <key> <field>`
    - 示例：`HGET user:1 name` → 返回 `"John"`
- **获取所有字段和值**：`HGETALL <key>`
    - 示例：`HGETALL user:1` → 返回 `["name", "John", "age", "30"]`

---

### 3. **List（列表）**
- **从左侧插入元素**：`LPUSH <key> <value>`
    - 示例：`LPUSH mylist "item1" "item2"`
- **从右侧插入元素**：`RPUSH <key> <value>`
    - 示例：`RPUSH mylist "item3" "item4"`
- **从左侧弹出元素**：`LPOP <key>`
    - 示例：`LPOP mylist` → 返回 `"item1"`
- **从右侧弹出元素**：`RPOP <key>`
    - 示例：`RPOP mylist` → 返回 `"item4"`
- **获取列表范围**：`LRANGE <key> <start> <end>`
    - 示例：`LRANGE mylist 0 -1` → 返回列表中的所有元素

---

### 4. **Set（集合）**
- **添加元素**：`SADD <key> <member>`
    - 示例：`SADD myset "apple" "banana" "cherry"`
- **检查元素是否存在**：`SISMEMBER <key> <member>`
    - 示例：`SISMEMBER myset "apple"` → 返回 `1`（存在）或 `0`（不存在）
- **获取所有元素**：`SMEMBERS <key>`
    - 示例：`SMEMBERS myset` → 返回 `["apple", "banana", "cherry"]`
- **移除元素**：`SREM <key> <member>`
    - 示例：`SREM myset "banana"`

---

### 5. **Sorted Set（有序集合）**
- **添加元素**：`ZADD <key> <score> <member>`
    - 示例：`ZADD myzset 1 "apple" 2 "banana" 3 "cherry"`
- **获取成员及其分数**：`ZRANGE <key> <start> <end> WITHSCORES`
    - 示例：`ZRANGE myzset 0 -1 WITHSCORES` → 返回 `[apple, 1, banana, 2, cherry, 3]`
- **移除成员**：`ZREM <key> <member>`
    - 示例：`ZREM myzset "banana"`
- **获取成员的分数**：`ZSCORE <key> <member>`
    - 示例：`ZSCORE myzset "apple"` → 返回 `1`

---

### 6. **Bitmap（位图）**
- **设置位图中的位**：`SETBIT <key> <offset> <value>`
    - 示例：`SETBIT mybitmap 0 1` → 设置第 0 位为 1
- **获取位图中的位**：`GETBIT <key> <offset>`
    - 示例：`GETBIT mybitmap 0` → 返回 `1`
- **统计位图中 1 的数量**：`BITCOUNT <key>`
    - 示例：`BITCOUNT mybitmap` → 返回位图中 1 的数量

---

### 7. **HyperLogLog（超日志）**
- **添加元素**：`PFADD <key> <element>`
    - 示例：`PFADD myhll "item1" "item2" "item3"`
- **获取近似基数**：`PFCOUNT <key>`
    - 示例：`PFCOUNT myhll` → 返回集合中不重复元素的近似数量

---

### 8. **Geo（地理位置）**
- **添加地理位置信息**：`GEOADD <key> <longitude> <latitude> <member>`
    - 示例：`GEOADD mygeo 116.39 39.91 "Beijing"` → 添加北京的经纬度
- **获取位置信息**：`GEOPOS <key> <member>`
    - 示例：`GEOPOS mygeo "Beijing"` → 返回 `[116.39, 39.91]`
- **计算两点距离**：`GEODIST <key> <member1> <member2>`
    - 示例：`GEODIST mygeo "Beijing" "Shanghai"` → 返回两地之间的距离（单位：米）

---

### 9. **Stream（流）**
- **添加消息**：`XADD <key> <ID> <field> <value>`
    - 示例：`XADD mystream * name "John" age 30` → 添加消息，`*` 表示自动生成 ID
- **读取消息**：`XREAD BLOCK <timeout> STREAMS <key> <ID>`
    - 示例：`XREAD BLOCK 0 STREAMS mystream 0` → 读取从 ID `0` 开始的所有消息
- **获取消息范围**：`XRANGE <key> <start> <end>`
    - 示例：`XRANGE mystream - +` → 返回所有消息（`-` 表示最小 ID，`+` 表示最大 ID）

---

