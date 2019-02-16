
export const numberOfPeople = (state = 0, action) => {
  console.log(`Reducer triggered with action: ${action}, previous state: ${state}`)
  switch (action.type) {
    case 'ADD_PERSON':
      return state + 1
    default:
      return state
  }
}
