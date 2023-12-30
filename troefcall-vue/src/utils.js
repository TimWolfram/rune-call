export function getShortName(name, maxLength = 20) {
  return name.length > maxLength ? name.substring(0, maxLength-2) + '...' : name;
}