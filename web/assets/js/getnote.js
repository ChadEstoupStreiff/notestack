

function load_note() {
    var pathArray = window.location.pathname.split('/');
    var note_id = pathArray[pathArray.length -1];

    document.getElementById("note_id").innerHTML = "ID: " + note_id;

    var xmlHttp = new XMLHttpRequest();
    xmlHttp.open("GET", "/api/note/" + note_id, false); // false for synchronous request
    xmlHttp.send(null);
    result = xmlHttp.responseText;

    document.getElementById("note").innerHTML = result;
}

document.addEventListener('DOMContentLoaded', function() {
    load_note();
 }, false);