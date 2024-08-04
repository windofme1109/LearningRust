# 1. Http 服务的注册方式

1. 注册方式如下：
![alt text](./imgs/actix-web注册service.png)


actix-web 提供了几种方式来注册路由：

1. `web::scope`
从字面意义上看，scope 有领域、范围的意思，这里表示一定范围的路由，这个一定范围指的是公共路由前缀，包含多个以公共路由前缀开头的路由
例如：/user/name、/user/age，/user/id 等，均是以 /user 开头，可以使用 scope 方法进行配置：web::scope("/user")
这里配置可以包括动态路径，但是不能包含末尾斜杠，即可以是：/user，但是不能是 /user/

返回值：Scope 是共享一个公共路径前缀的路由（Route）、资源（Resource）或其他服务（service）的集合
也就是可以在继续针对具体的路径配置路由或者是服务。

1. route 为一个指定的路径配置路由，第一个参数是路径，第二个参数是路由，也就是一个处理器（handler），即针对这个 path 做怎样的处理
route 是一个简化版的 Scope::service()
可以链式调用 route 方法


2. service 注册一个 Http 服务
service 的功能与 App::service 类似，核心都是注册 Http 服务，因此可以实现注册两种类型的 Http 服务：
   1. Resource 是资源表中的一个条目，对应于请求的 URL
   2. Scope 是具有公共根路径的一组资源
上面两个服务分别对应 web::resource 和 web::scope



`web::service`
这个方法是为特定路径创建原始服务
返回值： WebService，可以继续针对这个服务使用 guard、finish 等方法
web::service 提供的功能比较少，因此很少使用


`web::resource`
为特定路径创建新资源，而这个路径可以包括动态路径
返回值：Resource 是响应相同路径模式的路由（Route）的集合，要求每个资源至少有一个路由
通过 resource 方法创建的资源，后面配置的路由都是为这一个路径服务的

ServiceConfig 提供的 service 方法与 App::service 功能类似
核心功能都是注册一个 Http 服务，Http 服务是任何实现 HttpServiceFactory trait 的类型
Actix Web 提供了几种服务实现:
1. Resource 是资源表中的一个条目，对应于请求的 URL
2. Scope 是具有公共根路径的一组资源
上面两个服务分别对应 web::resource 和 web::scope

使用 ServiceConfig 中的 service 注册 Http 服务的目的是：可以将不同的服务配置放在对应的模块或者其他的库中
最后通过 App 提供的 configure 方法组合到一起