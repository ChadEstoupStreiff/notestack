function search_note() {
    var id_note = document.getElementById("id_note").value;

    if (id_note.length > 0) {
        window.location.href = "/note/" + id_note;
    }
}


function search_note_input(ele) {
    if (event.key === 'Enter') {
        search_note();
    }
}