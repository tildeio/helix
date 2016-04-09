function supersetOf(source, needle) {
  if (needle.length === 0) {
    return true;
  }

  if (source.length === 0) {
    return false;
  }

  let needlePosition = 0;
  let needleItem = needle[0];
  let needleLength = needle.length;

  for (let sourceIndex=0; sourceIndex<source.length; sourceIndex++) {
    let sourceItem = source[sourceIndex];

    if (sourceItem === needleItem) {
      needlePosition++;

      if (needlePosition >= needleLength) {
        return true;
      } else {
        needleItem = needle[needlePosition];
      }
    }
  }

  return false;
}
