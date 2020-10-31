(() => {
  const splits = document.body.textContent.split(/[,.]?\s+/gm)
  const counts = splits
    .filter(word => word.search(/[-–!-/:-@\[-`{}-~£↑,.·<>?\/\\\|«»0-9]/) == -1)
    .map(word => word.toLocaleLowerCase('pt'))
    .reduce((dict, word) => {
      if (dict.has(word)) {
        dict.set(word, dict.get(word) + 1)
      } else {
        dict.set(word, 1)
      }

      return dict
    }, new Map())

  return Array.from(counts.entries()).sort((a, b) => b[1] - a[1])
})()
