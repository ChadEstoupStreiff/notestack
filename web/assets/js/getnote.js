

function load_note() {
    var pathArray = window.location.pathname.split('/');
    var note_id = pathArray[pathArray.length -1];

    document.getElementById("note_id").innerHTML = "ID: " + note_id;

    var xmlHttp = new XMLHttpRequest();
    xmlHttp.open("GET", "/api/note/" + note_id, false); // false for synchronous request
    xmlHttp.send(null);
    result = xmlHttp.responseText;

    if (result.length > 0) {
        document.getElementById("note").innerHTML = result;
    } else {
        document.getElementById("note").innerHTML = "No note available at this ID";
    }
}

document.addEventListener('DOMContentLoaded', function() {
    load_note();
 }, false);