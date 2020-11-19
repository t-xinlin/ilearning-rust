use {
    hyper::{
        // Hyper中的其他类型，用于构建HTTP
        Body, Client, Request, Response, Server, Uri,

        // 该函数将闭包转换为实现了Hyper的Service trait的future
        // 它是从通用Request到Response的异步函数。
        service::service_fn,

        // 使用Hyper运行时可以运行future到完成的函数。
        rt::run,
    },
    futures::{
        // futures 0.1版本的一个扩展trait，添加了'.compat()'方法
        // 允许我们在0.1版本的futures中使用'.await'语法
        compat::Future01CompatExt,
        // 扩展trait在futures上提供了额外的方法在
        // `FutureExt` 添加了适用于所有futures的方法,
        // `TryFutureExt` 给futures添加了可以放回‘Result’类型的方法
        future::{FutureExt, TryFutureExt},
    },
    std::net::SocketAddr,
};

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    // 通常成功地返回一个包含友好问候的body的响应（response）
    Ok(Response::new(Body::from("hello, world!")))
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);
    // 创建绑定到提供的地址的服务器
    let serve_future = Server::bind(&addr)
        // 服务器请求使用 `async serve_req` 这个函数.
        // `serve` 方法采取闭包操作只要求返回的值实现‘Service’这个trait，
        //  `service_fn` 这个函数正好返回一个实现'Service' 这个trait的值
        // 该方法也是采用闭包操作，返回的是一个响应future的请求
        // 为了使用Hyper的serve_req这个函数，我们必须用box把它包裹起来
        // 并且用compat方法让他获得0.1futures的兼容性（Hyper还是0.1的futures，所以显得麻烦）
        .serve(|| service_fn(|req|
            serve_req(req).boxed().compat()
        ));
    // 等待服务完成服务或者因为某个错误而退出
    // 如果一个错误出现，输出它到stderr
    if let Err(e) = serve_future.compat().await {
        eprintln!("server error: {}", e);
    }
}

fn main() {
    // 设置地址以运行我们的套接字
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // 调用 run_server 函数, 将返回一个future
    // 与每一个`async fn`一样, 对于`run_server`做任何事情，
    // 返回的future需要运行
    //   额外地，我们需要将返回的future从futures0.3转换成0.1版本的future
    let futures_03_future = run_server(addr);
    let futures_01_future =
        futures_03_future.unit_error().boxed().compat();
    // 最后我们用Hyper提供的run方法运行我们的future直到完成
    run(futures_01_future);
}
