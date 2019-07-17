Rust.

接口
- [x] 3.1创建钱包
- [ ] 4.1创建Remote对象
- [x] 4.2创建连接
- [ ] 4.3关闭连接
- [x] 4.4请求底层服务器信息
- [x] 4.5获取最新账本信息
- [x] 4.6获取某一账本具体信息
- [x] 4.7查询某一交易具体信息
- [x] 4.8请求账号信息
- [x] 4.9获得账号可接收和发送的货币
- [x] 4.10获得账号关系
- [x] 4.11获得账号挂单
- [x] 4.12获得账号交易列表
- [x] 4.13获得市场挂单列表
- [x] 4.14获得挂单佣金设置信息
- [x] 4.15支付 
    - [x]    4.15.1创建支付对象 
    - [x]    4.15.2传入密钥
    - [x]    4.15.3设置备注
    - [x]    4.15.4提交支付
- [x] 4.16设置关系
    - [x]    4.16.1创建关系对象
    - [x]    4.16.2传入密钥
    - [x]    4.16.3关系设置
- [ ] 4.17设置账号属性 -----------待完善
    - [ ]    4.17.1创建属性对象
    - [ ]    4.17.2传入密钥
    - [ ]    4.17.3属性设置
- [x] 4.18挂单
    - [x]    4.18.1创建挂单对象
    - [x]    4.18.2传入密钥
    - [x]    4.18.3提交挂单
- [x] 4.19取消挂单
    - [x]    4.19.1创建取消挂单对象
    - [x]    4.19.2传入密钥
    - [x]    4.19.3取消挂单

待实现工具方法：  
- [x] 字符串转16进制： fn string_to_hex(s: String) -> String;  
- [x] 本地签名


遗留问题:
- [x] 1，完善Amount数据结构
- [ ] 2，TakerGets和TakerPays在不同情况下的数据类型 （Amount or String）
- [ ] 3，新老服务器接口的更新（涉及到app/brokerage等相关字段）
- [x] 4，添加[[example]]
- [ ] 5，添加[[test]]
- [ ] 6，挂单接口中flags字段根据Sell/Buy的设置
- [ ] 7，代码架构调整
- [ ] 8，参数检查
- [ ] 9，异常处理
- [ ] 10，添加ed25519模块
- [ ] 11, 支持签名算法[ed25519/secp256k1]可配置
- [ ] 12, 交易类型【TX_TRANSACTION_TYPE】参数，根据上边代码赋值。
- [x] 13, tx_json对Amount，Account等String类型的序列化问题。
- [x] 14, 交易blob的签名。
- [ ] 15, 修改pub fn prepare(tx_json: TxJson)，===》 pub fn prepare(tx_json: &mut TxJson)，在原有基础上修改txjson。
- [ ] 16, 重构TypeObject compare方法。
- [x] 17, base/Amount 和 common/Amount映射！！！
- [ ] 18, LocalSignX / TransactionX 
