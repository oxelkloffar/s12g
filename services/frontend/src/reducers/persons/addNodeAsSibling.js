import { addChild, addParent, addSibling, addPartner } from './utils'

export default function addNodeAsSibling(newId, newState, siblingId) {
	// add possible parents
	newState
		.filter(node => node.id === siblingId)
		.flatMap(node => node.parents)
		.map(node => node.id)
		// filter out duplicates:
		.filter((value, index, self) => self.indexOf(value) === index)
		.forEach(parentId => {
			newState = addChild(newState, parentId, newId)
			newState = addParent(newState, newId, parentId)
		})

	// add siblings
	newState
		.filter(node => node.id === siblingId)
		.flatMap(node => node.siblings)
		.map(node => node.id)
		// filter out duplicates:
		.filter((value, index, self) => self.indexOf(value) === index)
		.forEach(siblingId => {
			newState = addSibling(newState, siblingId, newId)
			newState = addSibling(newState, newId, siblingId)
		})

	return newState
}
