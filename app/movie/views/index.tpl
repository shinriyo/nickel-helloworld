﻿<!DOCTYPE html>
<html data-ng-app="movieApp">
<head lang="en">
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <base href="/"/>
    <title>The Movie App</title>
    <link rel="stylesheet" type="text/css" href="css/bootstrap.min.css"/>
    <link rel="stylesheet" type="text/css" href="css/app.css"/>
</head>
<body>
    <nav class="navbar navbar-default" role="navigation">
        <div class="container-fluid">
            <div class="navbar-header">
                <a class="navbar-brand" ui-sref="movies">The Movie App</a>
            </div>
            <div class="collapse navbar-collapse">
                <ul class="nav navbar-nav">
                    <li class="active"><a ui-sref="movies">Home</a></li>
                </ul>
            </div>
        </div>
    </nav>
    <div class="container">
        <div class="row top-buffer">
            <div class="col-xs-8 col-xs-offset-2">
                <div ui-view></div>
            </div>
        </div>
    </div>
    <script type="text/javascript" src="lib/angular.min.js"></script>
    <script type="text/javascript" src="movie/js/app.js"></script>
    <script type="text/javascript" src="movie/js/controllers.js"></script>
    <script type="text/javascript" src="movie/js/services.js"></script>
    <script type="text/javascript" src="movie/js/directives.js"></script>
    <script type="text/javascript" src="movie/js/filters.js"></script>
    <script type="text/javascript" src="lib/angular-ui-router.min.js"></script>
    <script type="text/javascript" src="lib/angular-resource.min.js"></script>
</body>
</html>