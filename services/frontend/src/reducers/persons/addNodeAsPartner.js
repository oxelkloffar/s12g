import { addChild, addParent, addSibling, addPartner } from './utils'

export default function addNodeAsPartner(newId, newState, partnerId) {
	// add partner connections
	newState = addPartner(newState, partnerId, newId)
	newState = addPartner(newState, newId, partnerId)

	// add possible children
	newState
		.filter(node => node.id === partnerId)
		.flatMap(node => node.children)
		.map(node => node.id)
		// filter out duplicates:
		.filter((value, index, self) => self.indexOf(value) === index)
		.forEach(childId => {
			newState = addChild(newState, newId, childId)
			newState = addParent(newState, childId, newId)
		})

	return newState
}
