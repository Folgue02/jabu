/*
function goToArtifact(event) {
    let artifact_name = event.target.textContent;
    window.location.href  = "./artifact.html?id=" + encodeURI(artifact_name);
}*/

function goToArtifact(author, artifact_id) {
    window.location.href = `./artifact.html?author=${encodeURI(artifact_id)}&id=${encodeURI(author)}`;
}

function redirectToSearch() {
    let queryTerm = document.getElementById("searchbar").value;
    window.location.href = "./search.html?search_term=" + encodeURI(queryTerm);
}

// This is for the searchbar
document.getElementById("searchbar").addEventListener("keypress", (event) => {
    console.log(event.key);
    if (event.key == "Enter")
        redirectToSearch();
});
