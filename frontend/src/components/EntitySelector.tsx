import React from 'react';
import Checkbox from '@mui/material/Checkbox';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemIcon from '@mui/material/ListItemIcon';
import ListItemText from '@mui/material/ListItemText';
import Typography from '@mui/material/Typography';

/**
 * Props for the EntitySelector component
 */
interface Props {
  tables: string[];
  selected: string[];
  onChange: (selected: string[]) => void;
}

/**
 * Component for selecting tables/entities to expose as API endpoints
 */
const EntitySelector: React.FC<Props> = ({ tables, selected, onChange }) => {
  /**
   * Handles toggling a table selection
   */
  const handleToggle = (table: string) => {
    const currentIndex = selected.indexOf(table);
    const newSelected = [...selected];
    if (currentIndex === -1) {
      newSelected.push(table);
    } else {
      newSelected.splice(currentIndex, 1);
    }
    onChange(newSelected);
  };

  return (
    <div style={{ maxWidth: 400, margin: '2rem auto' }}>
      <Typography variant="h6" gutterBottom>Select tables to expose as API endpoints</Typography>
      <List>
        {tables.map((table) => (
          <ListItem key={table} disablePadding>
            <ListItemButton onClick={() => handleToggle(table)}>
              <ListItemIcon>
                <Checkbox
                  edge="start"
                  checked={selected.indexOf(table) !== -1}
                  tabIndex={-1}
                  disableRipple
                  inputProps={{ 'aria-labelledby': table }}
                />
              </ListItemIcon>
              <ListItemText id={table} primary={table} />
            </ListItemButton>
          </ListItem>
        ))}
      </List>
    </div>
  );
};

export default EntitySelector; 