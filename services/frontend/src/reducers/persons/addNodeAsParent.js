import { addChild, addParent, addSibling, addPartner } from './utils'

export default function addNodeAsParent(newId, newState, childId) {
	// add children to new node
	newState = addChild(newState, newId, childId)
	newState = addParent(newState, childId, newId)

	// add possible other children
	newState
		.filter(node => node.id === childId)
		.flatMap(node => node.siblings)
		.map(node => node.id)
		// filter out duplicates:
		.filter((value, index, self) => self.indexOf(value) === index)
		.forEach(childId => {
			newState = addChild(newState, newId, childId)
			newState = addParent(newState, childId, newId)
		})

	// add possible partner(s?)
	newState
		.filter(node => node.id === childId)
		.flatMap(node => node.parents)
		.map(node => node.id)
		// filter out duplicates:
		.filter((value, index, self) => self.indexOf(value) === index)
		.filter(node => node.id !== newId)
		.forEach(partnerId => {
			newState = addPartner(newState, partnerId, newId)
			newState = addPartner(newState, newId, partnerId)
		})

	return newState
}
