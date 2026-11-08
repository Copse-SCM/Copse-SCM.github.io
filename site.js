// Open at the top (unless a link points further down), and add copy buttons.
if ("scrollRestoration" in history) history.scrollRestoration = "manual";
window.addEventListener("load", function () {
  if (!location.hash) window.scrollTo(0, 0);
});
document.querySelectorAll("pre.copyable").forEach(function (pre) {
  var b = document.createElement("button");
  b.className = "copy";
  b.type = "button";
  b.textContent = "Copy";
  b.addEventListener("click", function () {
    var text = pre.querySelector("code").innerText.replace(/\s+#.*$/gm, "");
    navigator.clipboard.writeText(text).then(function () {
      b.textContent = "Copied";
      setTimeout(function () { b.textContent = "Copy"; }, 1400);
    });
  });
  pre.appendChild(b);
});
