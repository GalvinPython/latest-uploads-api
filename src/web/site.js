const showOptionsCheckbox = document.getElementById('show-options');
const optionalFields = document.getElementById('optional-fields');
const videoList = document.getElementById('video-list');

showOptionsCheckbox.addEventListener('change', () => {
    optionalFields.classList.toggle('hidden', !showOptionsCheckbox.checked);
});

const form = document.getElementById('fetch-videos-form');
form.addEventListener('submit', async (event) => {
    event.preventDefault();

    const channelId = document.getElementById('channel-id').value;
    const videoCount = document.getElementById('video-count').value || 5;
    const typeDoc = document.getElementById('type').value || 'all';
    const type = typeDoc === 'all' ? 'all' :
        typeDoc === 'shorts' ? 'shorts' :
            typeDoc === 'streams' ? 'live' :
                'videos';

    if (!channelId) {
        alert('Please enter a YouTube Channel ID.');
        return;
    }

    try {
        const response = await fetch(`/get/${channelId}?type=${type}&maxresults=${videoCount}`);
        if (!response.ok) throw new Error('Failed to fetch videos');

        const data = await response.json();
        renderVideos(data);
    } catch (error) {
        console.error('Error fetching videos:', error);
        alert('An error occurred while fetching videos.');
    }
});

function renderVideos(videos) {
    videoList.innerHTML = '';
    videos.forEach(video => {
        const videoCard = document.createElement('a');
        videoCard.href = `https://www.youtube.com/watch?v=${video.videoId}`;
        videoCard.target = '_blank';
        videoCard.className =
            "block bg-white dark:bg-gray-800 rounded-lg shadow hover:shadow-lg transition-shadow overflow-hidden";

        videoCard.innerHTML = `
                    <img src="https://i.ytimg.com/vi/${video.videoId}/hqdefault.jpg" alt="${video.title}" class="w-full h-40 object-cover">
                    <div class="p-4">
                        <h2 class="text-lg font-semibold text-blue-600 dark:text-blue-400 hover:underline line-clamp-2">${video.title}</h2>
                        <p class="text-xs text-gray-500 mt-1">${new Date(video.publishedAt).toLocaleString()}</p>
                    </div>
                `;
        videoList.appendChild(videoCard);
    });
}