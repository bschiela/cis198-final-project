var searchIndex = {};
searchIndex['ecs_client'] = {"items":[[0,"ecs_client","ecs_client","This module contains the ECSClient which can be used to interact with Amazon ECS's API.",null,null],[3,"ECSClient","ecs_client::ecs_client","",null,null],[12,"region","","",0,null],[12,"client","","",0,null],[11,"new","","creates a new ECSClient for the specified Region",0,{"inputs":[{"name":"ecsclient"},{"name":"region"}],"output":{"name":"ecsclient"}}],[11,"set_region","","sets the Region to which the client sends requests",0,{"inputs":[{"name":"ecsclient"},{"name":"region"}],"output":null}],[0,"region","ecs_client","An enum defining the regions in which Amazon ECS is supported.",null,null],[4,"Region","ecs_client::region","",null,null],[13,"USEast1","","",1,null],[13,"USWest1","","",1,null],[13,"USWest2","","",1,null],[13,"EUWest1","","",1,null],[13,"EUCentral1","","",1,null],[13,"APNortheast1","","",1,null],[13,"APSoutheast1","","",1,null],[13,"APSoutheast2","","",1,null],[11,"clone","","",1,{"inputs":[{"name":"region"}],"output":{"name":"region"}}],[11,"fmt","","",1,{"inputs":[{"name":"region"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"request","ecs_client","A Signature Version 4 Signable AWS Request to ECS",null,null],[3,"ECSRequest","ecs_client::request","",null,null],[12,"region","","",2,null],[12,"req_builder","","",2,null],[17,"AMZ_SUBLEVEL_CONTENT_TYPE","","The MIME sublevel content type of an ECS HTTP request body",null,null],[11,"new","","Creates a new ECSRequest destined for the specified Region",2,{"inputs":[{"name":"ecsrequest"},{"name":"region"},{"name":"requestbuilder"}],"output":{"name":"ecsrequest"}}],[11,"set_host_header","","Sets the Host header",2,{"inputs":[{"name":"ecsrequest"}],"output":null}],[11,"set_accept_encoding_header","","Sets the Accept-Encoding header",2,{"inputs":[{"name":"ecsrequest"}],"output":null}],[11,"set_x_amz_target_header","","Sets the X-Amz-Target header",2,{"inputs":[{"name":"ecsrequest"}],"output":null}],[11,"set_x_amz_date_header","","Sets the X-Amz-Date header",2,{"inputs":[{"name":"ecsrequest"}],"output":null}],[11,"set_content_type_header","","Sets the Content-Type header",2,{"inputs":[{"name":"ecsrequest"}],"output":null}],[11,"set_authorization_header","","Sets the Authorization header\nnote: may be replaced by a sign() or sign_and_send() function",2,{"inputs":[{"name":"ecsrequest"}],"output":null}],[0,"params","ecs_client","Defines structs which contain the relevant parameters for each type of request to Amazon ECS.\nEach can be serialized to a json blob and set as the body of the HTTP request.",null,null],[3,"ListClustersParams","ecs_client::params","allowed parameters for a ListClusters request",null,null],[12,"max_results","","must be between 1 and 100, inclusive; defaults to 100 if missing or invalid",3,null],[12,"next_token","","an optional token returned by a previous ListClusters request indicating where to start\nthe next page of paginated output (if there are more results than max_results)",3,null]],"paths":[[3,"ECSClient"],[4,"Region"],[3,"ECSRequest"],[3,"ListClustersParams"]]};
initSearch(searchIndex);
