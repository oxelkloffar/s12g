import nodes from '../data/sampledata.json'

export const numberOfPeople = (state = nodes, action) => {
  console.log(`Reducer triggered with action: ${action}, previous state: ${state}`)

  switch (action.type) {
    case 'ADD_PERSON':
      console.log(action.name)
      return [...state, action.name]

    case 'SET_NODE_NAME':
      return state.map(node => {
        if (node.id === action.payload.id) {
          return { ...node, name: action.payload.name }
        }
        return node
      })

    default:
      return state
  }
}
