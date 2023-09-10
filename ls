[1mdiff --git a/src/templates/red/home.html b/src/templates/red/home.html[m
[1mindex 5976f6c..2c4f89a 100644[m
[1m--- a/src/templates/red/home.html[m
[1m+++ b/src/templates/red/home.html[m
[36m@@ -99,7 +99,6 @@[m
 [m
     function open_file(uuid){[m
         var e = $(`#tab_file_${uuid}`)[m
[31m-        console.log(e);[m
         if( e.length > 0 ){[m
             remove_from_classlist('btn_file_', ['active']);[m
             remove_from_classlist('tab_file_', ['active', 'show']);[m
[36m@@ -136,6 +135,9 @@[m
             automaticLayout: true,[m
             theme: 'vs-dark'[m
         });[m
[32m+[m[32m        $(`#${tab_id}`).data('editor', editor);[m
[32m+[m[32m        console.log($(`#${tab_id}`).data('editor'));[m
[32m+[m
     }[m
 [m
     function change_directory(directory){[m
[36m@@ -168,7 +170,7 @@[m
             ${filename}[m
             </div>[m
             <div class="col-sm-3 gx-7 d-flex flex-row-reverse" style="">[m
[31m-            <button type="button" class="btn p-0" aria-label="Close">X</button>[m
[32m+[m[32m            <button type="button" class="btn p-0" aria-label="Close" onclick="close_editor('${file_uuid}')">X</button>[m
             </div>[m
             </div>[m
 [m
[36m@@ -177,10 +179,15 @@[m
     }[m
 [m
     function session_expired(){[m
[31m-        alert("Your session has expired, you will be redirect to login page.");[m
[32m+[m[32m        alert('Your session has expired, you will be redirect to login page.');[m
         location.reload();[m
     }[m
 [m
[32m+[m[32m    function close_editor(uuid){[m
[32m+[m[32m        var tab_file = $(`#tab_file_${uuid}`);[m
[32m+[m[32m        console.log(tab_file.data('editor'));[m
[32m+[m[32m    }[m
[32m+[m
 </script>[m
 {% endblock scripts %}[m
 [m
