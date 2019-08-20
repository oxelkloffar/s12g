import nodes from '../data/sampledata.json'

export default function persons(state = nodes, action) {
  console.log(`Reducer triggered with action: ${action}, previous state: ${state}`)

  switch (action.type) {
    case 'ADD_PERSON':
      console.log(action.name)
      return [...state, action.name]

    case 'SET_PERSON_NAME':
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
