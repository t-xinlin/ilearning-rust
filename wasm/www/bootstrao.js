<!DOCTYPE html>
<html>

<head>
  <meta charset="utf-8">
  <title>Hello wasm-pack!</title>
  <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.2.3/dist/css/bootstrap.min.css" rel="stylesheet"
        integrity="sha384-rbsA2VBKQhggwzxH7pPCaAqO46MgnOM80zW1RWuH61DGLwZJEdK2Kadq2F9CUG65" crossorigin="anonymous" />
</head>

<body>
<noscript>This page contains webassembly and javascript content, please enable javascript in your browser.</noscript>
<nav class="navbar navbar-dark bg-primary">
  <div class="container-fluid">
    <a class="navbar-brand" href="#">Wasm-client</a>
  </div>
</nav>
<div class="m-3" style="height: 600px">
  <div class="row">
    <div class="col">
      <div class="card border-info mb-3">
        <div class="card-header">课程列表</div>
        <!-- <div class="card-body">
          <button type="button" class="btn btn-primary">Add</button>
        </div> -->
        <table class="table talbe-hover table-bordered table-sm">
          <thead>
          <tr>
            <th scope="col">ID</th>
            <th scope="col">名称</th>
            <th scope="col">时间</th>
            <th scope="col">简介</th>
            <th scope="col">操作</th>
          </tr>
          </thead>
          <tbody id="left-tbody"></tbody>
        </table>
        <div id="left"></div>
      </div>
    </div>
    <div class="col">
      <div class="card border-info mb-3">
        <div class="card-header">添加课程</div>
        <div class="card-body">
          <form class="row g-3 needs-validation" id="form">
            <div class="mb-3">
              <label for="name" class="form-label">课程名称</label>
              <input type="name" class="form-control" id="name" required placeholder="课程名称" />
              <div class="invalid-feedback">
                请填写课程名！
              </div>
            </div>
            <div class="mb-3">
              <label for="description" class="form-label">课程简介</label>
              <textarea class="form-control" id="description" rows="3"></textarea>
            </div>
            <div class="col-12">
              <button type="submit" class="btn btn-primary">提交</button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</div>
<script src="./bootstrap.js"></script>
</body>

</html>

