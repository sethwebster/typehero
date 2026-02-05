// Filter adults and map to names
function getAdultNames(users) {
  return users.filter(user => user.age >= 18).map(user => user.name);
}
