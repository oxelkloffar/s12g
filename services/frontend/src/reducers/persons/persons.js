import nodes from '../../data/sampledata.json'
import addNodeAsChild from './addNodeAsChild'
import addNodeAsParent from './addNodeAsParent'
import addNodeAsSibling from './addNodeAsSibling'
import addNodeAsPartner from './addNodeAsPartner'

export default function persons(state = nodes, action) {
  switch (action.type) {
    case 'FINISH_ADDING_RELATIVE':
      const newId = action.payload.name
      const newState = [
        ...state,
        {
          id: newId,
          name: action.payload.name,
          gender: '',
          parents: [],
          children: [],
          siblings: [],
          spouses: []
        }
      ]

      switch (action.payload.relation) {
        // add a new child node
        case 'child':
          return addNodeAsChild(newId, newState, action.payload.relatedTo)

        // add a new parent node
        case 'parent':
          return addNodeAsParent(newId, newState, action.payload.relatedTo)

        // add a new sibling node
        case 'sibling':
          return addNodeAsSibling(newId, newState, action.payload.relatedTo)

        // add a new partner node
        case 'partner':
          return addNodeAsPartner(newId, newState, action.payload.relatedTo)

        default:
          return state
      }

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
