(function () {
  function copyText(text, button) {
    var done = function () {
      var previous = button.textContent;
      button.textContent = "Copied";
      button.classList.add("copied");
      window.setTimeout(function () {
        button.textContent = previous;
        button.classList.remove("copied");
      }, 1600);
    };

    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(text).then(done);
      return;
    }

    var textarea = document.createElement("textarea");
    textarea.value = text;
    textarea.setAttribute("readonly", "");
    textarea.style.position = "absolute";
    textarea.style.left = "-9999px";
    document.body.appendChild(textarea);
    textarea.select();
    document.execCommand("copy");
    document.body.removeChild(textarea);
    done();
  }

  function enhanceCodeBlocks() {
    var blocks = document.querySelectorAll(".highlight");
    blocks.forEach(function (block) {
      if (block.parentElement && block.parentElement.classList.contains("code-block")) {
        return;
      }

      var wrapper = document.createElement("div");
      wrapper.className = "code-block";
      block.parentNode.insertBefore(wrapper, block);
      wrapper.appendChild(block);

      var code = block.querySelector("code");
      if (!code) {
        return;
      }

      var button = document.createElement("button");
      button.type = "button";
      button.className = "copy-code";
      button.textContent = "Copy";
      button.setAttribute("aria-label", "Copy code block");
      button.addEventListener("click", function () {
        copyText(code.textContent, button);
      });
      wrapper.appendChild(button);
    });
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", enhanceCodeBlocks);
  } else {
    enhanceCodeBlocks();
  }
})();
