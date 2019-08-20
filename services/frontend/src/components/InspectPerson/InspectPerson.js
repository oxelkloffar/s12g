import React from 'react'
import { useSelector, useDispatch } from 'react-redux'
import styles from './InspectPerson.module.css'

function InspectPerson(){
	const dispatch = useDispatch()
	const selectedId = useSelector(state => state.selectedPerson)
	const selectedPerson = useSelector(state => state.persons.filter(p => p.id === selectedId))

	if (selectedId === null) {
		return <></>
	}

	const person = selectedPerson[0]

	return(
		<div className={styles.bg} onClick={() => dispatch({ type: 'DESELECTED_PERSON' })}>
			<div className={styles.modal} onClick={(e) => { e.stopPropagation(); dispatch({ type: 'SET_PERSON_NAME', payload:{id:person.id, name:person.name+'!'} }) }}>
				{person.name}
			</div>
		</div>
	)
}

export default InspectPerson
