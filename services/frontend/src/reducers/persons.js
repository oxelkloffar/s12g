import nodes from '../data/sampledata.json'

// add a child relationship to a parent node
const addChild = (state, parentId, childId) =>
  state.map(node => {
    if (node.id === parentId) {
      return {
        ...node,
        children: [
          ...node.children,
          {
            id: childId,
            type: 'blood'
          }
        ]
      }
    }
    return node
  })

// add parent relationship for a child node
const addParent = (state, childId, parentId) =>
  state.map(node => {
    if (node.id === childId) {
      return {
        ...node,
        parents: [
          ...node.parents,
          {
            id: parentId,
            type: 'blood'
          }
        ]
      }
    }
    return node
  })

// add sibling relationships to related nodes
const addSibling = (state, siblingsIds, newSiblingId) =>
  state.map(node => {
    // add new node as sibling to all previously existing siblings
    if (siblingsIds.includes(node.id)) {
      return {
        ...node,
        siblings: [
          ...node.siblings,
          {
            id: newSiblingId,
            type: 'blood'
          }
        ]
      }
    }
    // add all previously existing siblings to new node
    else if (node.id === newSiblingId) {
      return {
        ...node,
        siblings: siblingsIds.map(id => ({
          id,
          type: 'blood'
        }))
      }
    }
    return node
  })

// add partner relationship for new node
const addPartner = (state, partnerId, newPartnerId) =>
  state.map(node => {
    return node
  })



export default function persons(state = nodes, action) {
  switch (action.type) {
    case 'ADD_PERSON':
      console.log(action.name)
      return [...state, action.name]

    case 'FINISH_ADDING_RELATIVE':
      const parentId = action.payload.relatedTo
      const newId = action.payload.name
      let newState = [
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
          // add first parent to new node
          newState = addChild(newState, parentId, newId)
          newState = addParent(newState, newId, parentId)

          // add possible other parent(s?)
          newState
            .filter(node => node.id === parentId)
            .flatMap(node => node.spouses)
            .map(node => node.id)
            .forEach(parentId => {
              newState = addChild(newState, parentId, newId)
              newState = addParent(newState, newId, parentId)
            })

          // add siblings
          const siblingsIds = state
            .filter(person =>
              person.parents
                .map(parent => parent.id)
                .includes(parentId)
            )
            .map(person => person.id)
          newState = addSibling(newState, siblingsIds, newId)

        // add a new parent node
        case 'parent':

        // add a new sibling node
        case 'sibling':

        // add a new partner node
        case 'partner':
      }

      return newState

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
