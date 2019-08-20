import React from 'react'
import classNames from 'classnames'
import { useDispatch } from 'react-redux'
import styles from './FamilyNode.module.css'

export default React.memo(
  function FamilyNode({ node, isRoot, onSubClick, style }) {
    const dispatch = useDispatch()
    return (
      <div className={styles.root} style={style}>
        <div
          className={classNames(
            styles.inner,
            styles[node.gender],
            isRoot && styles.isRoot,
          )}
          onClick={() => dispatch( {type:'SELECT_PERSON', payload:{id: node.id}} )}
        >
          <span>{node.name}</span>
        </div>
        {node.hasSubTree && (
          <div
            className={classNames(styles.sub, styles[node.gender])}
            onClick={() => onSubClick(node.id)}
          />
        )}
      </div>
    );
  }
);
