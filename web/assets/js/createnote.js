function submit_note_form() {
    let id_note = document.getElementById("id_note").value;
    let note = document.getElementById("note").value;

    if (note.length > 0) {
        var http = new XMLHttpRequest();
        http.open('POST', "/api/create", true);
        http.setRequestHeader('Content-type', 'application/x-www-form-urlencoded');
    }
    http.onreadystatechange = function() {//Call a function when the state changes.
        if(http.readyState == 4 && http.status == 200) {
            let answer = http.responseText;
            if (answer.startsWith("Success:")) {
                let note_id = answer.split(":")[1];
                window.location.href = "/note/" + note_id;
            } else {
                alert('Error creating note: ' + answer);
            }
        }
    }
    http.send("id=" + id_note + "&note=" + note + "");
}