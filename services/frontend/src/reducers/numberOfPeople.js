
export const numberOfPeople = (state = [], action) => {
  console.log(`Reducer triggered with action: ${action}, previous state: ${state}`)

  switch (action.type) {
    case 'ADD_PERSON':
      console.log(action.name)
      return [...state, action.name]

    default:
      return state
  }
}
