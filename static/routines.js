function remove_from_classlist(prefix, to_remove){
    var elements = $(`[id^="${prefix}"]`);
    for(var i=0 ; i < elements.length ; i++ ){
        for(var j=0 ; j < to_remove.length; ++j ){
            elements[i].classList.remove(to_remove[j]);
        }
    }
}




