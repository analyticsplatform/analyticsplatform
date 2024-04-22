// app/profile.tsx
import SidebarNav from './menu';

async function fetchProfileData() {
  try {
    const response = await fetch('http://localhost:3000/profile', { cache: 'no-store' });
    const data = await response.json();
    return data;
  } catch (error) {
    console.error('Error fetching profile data:', error);
    throw error;
  }
}

export const Profile = async ({ children }) => {
  const profileData = await fetchProfileData();
  return (
    <>
      <SidebarNav />
      <div className="flex-1 sb:ml-64">{children}</div>
      {/* Render the profile data */}
    </>
  );
};
