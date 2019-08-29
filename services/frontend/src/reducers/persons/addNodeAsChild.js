import { addChild, addParent, addSibling, addPartner } from './utils'

export default function addNodeAsChild(newId, newState, parentId) {
	// add first parent to new node
	newState = addChild(newState, parentId, newId)
	newState = addParent(newState, newId, parentId)

	// add possible other parent(s?)
	newState
		.filter(node => node.id === parentId)
		.flatMap(node => node.spouses)
		.map(node => node.id)
		// filter out duplicates:
		.filter((value, index, self) => self.indexOf(value) === index)
		.forEach(parentId => {
			newState = addChild(newState, parentId, newId)
			newState = addParent(newState, newId, parentId)
		})

	// add siblings
	newState
		.filter(node => node.id === parentId)
		.flatMap(node => node.children)
		.map(node => node.id)
		// filter out duplicates:
		.filter((value, index, self) => self.indexOf(value) === index)
		.filter(node => node.id !== newId)
		.forEach(siblingId => {
			newState = addSibling(newState, siblingId, newId)
			newState = addSibling(newState, newId, siblingId)
		})

	return newState
}
