// add a child relationship to a parent node
export const addChild = (state, parentId, childId) =>
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
export const addParent = (state, childId, parentId) =>
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


// add sibling relationship to node
export const addSibling = (state, siblingId, newSiblingId) =>
  state.map(node => {
    if (node.id === siblingId) {
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
    return node
  })


// add partner relationship for node
export const addPartner = (state, partnerId, newPartnerId) =>
  state.map(node => {
    if (node.id === partnerId) {
      return {
        ...node,
        spouses: [
          ...node.spouses,
          {
            id: newPartnerId,
            type: 'blood'
          }
        ]
      }
    }
    return node
	})
