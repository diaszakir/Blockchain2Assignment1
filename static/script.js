async function fetchNews() {
    const query = document.getElementById("cryptoInput").value;
    const response = await fetch(`/news?query=${query}`);
    const news = await response.json();
    
    const container = document.getElementById("news-container");
    container.innerHTML = "";
    
    if (news.length === 0) {
      container.innerHTML = "<p>No news found for this cryptocurrency.</p>";
      return;
    }
    
    news.forEach(article => {
      const item = document.createElement("div");
      item.classList.add("news-item");
      item.innerHTML = `
        <h3>${article.title}</h3>
        <p><strong>Source:</strong> ${article.source}</p>
        <p><strong>Date:</strong> ${new Date(article.date).toLocaleString()}</p>
        <a href="${article.url}" target="_blank">Read more</a>
      `;
      container.appendChild(item);
    });
  }
  