function redirectToSearch() {
    let queryTerm = document.getElementById("searchbar").value;
    window.location.href = "./search.html?search_term=" + encodeURI(queryTerm);
}

document.getElementById("searchbar").addEventListener("keypress", (event) => {
    console.log(event.key);
    if (event.key == "Enter") 
        redirectToSearch();
});
